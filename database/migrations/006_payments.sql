CREATE TYPE payment_method AS ENUM ('cash', 'transfer', 'card', 'discount');

CREATE TABLE IF NOT EXISTS payments (
    amount_cents    INTEGER        NOT NULL,
    created_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    id              UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    notes           VARCHAR(500),
    paid_at         TIMESTAMPTZ    NOT NULL,
    payment_method  payment_method NOT NULL,
    student_id      UUID           NOT NULL REFERENCES students(id),

    CONSTRAINT payments_amount_positive
        CHECK (amount_cents > 0)
);

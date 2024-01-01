-- Add migration script here
ALTER TABLE subscriptions
    ALTER COLUMN subscribed_at TYPE TIMESTAMPTZ USING subscribed_at::timestamptz;
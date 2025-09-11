ALTER TABLE film ADD metadata JSONB;
UPDATE public.film SET metadata = '{"foo": 123, "bar": "baz"}' WHERE film_id = 1;
UPDATE public.film SET metadata = '{"foo": 456, "bar": "boo"}' WHERE film_id = 2;
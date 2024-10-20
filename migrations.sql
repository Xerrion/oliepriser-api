CREATE TABLE IF NOT EXISTS providers
(
    id            SERIAL PRIMARY KEY,
    name          VARCHAR(255) NOT NULL,
    url           VARCHAR(255) NOT NULL,
    html_element  VARCHAR(255) NOT NULL,
    created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_updated  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_accessed TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS delivery_zones
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS provider_delivery_zones
(
    provider_id INT NOT NULL REFERENCES providers (id) ON DELETE CASCADE,
    zone_id     INT NOT NULL REFERENCES delivery_zones (id) ON DELETE CASCADE,
    PRIMARY KEY (provider_id, zone_id)
);

CREATE TABLE IF NOT EXISTS oil_prices
(
    id          SERIAL PRIMARY KEY,
    price       DOUBLE PRECISION NOT NULL,
    provider_id INT              NOT NULL REFERENCES providers (id) ON DELETE CASCADE,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS users
(
    id            SERIAL PRIMARY KEY,
    client_id     VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255)        NOT NULL,
    created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS scraping_runs
(
    id         SERIAL PRIMARY KEY,
    start_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    end_time   TIMESTAMP
);

DO
$$
    BEGIN
        IF EXISTS (SELECT 1
                   FROM information_schema.columns
                   WHERE table_name = 'oil_prices'
                     AND column_name = 'price'
                     AND data_type = 'double precision') THEN
            RAISE NOTICE 'Column price is already of type FLOAT8.';
        ELSE
            EXECUTE 'ALTER TABLE oil_prices ALTER COLUMN price TYPE FLOAT8 USING price::FLOAT8;';
            RAISE NOTICE 'Column price type changed to FLOAT8.';
        END IF;
    END
$$;

DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1
                       FROM information_schema.columns
                       WHERE table_name = 'providers'
                         AND column_name = 'last_accessed') THEN
            EXECUTE 'ALTER TABLE providers ADD COLUMN last_accessed TIMESTAMP DEFAULT CURRENT_TIMESTAMP;';
            RAISE NOTICE 'Column last_accessed added.';
        ELSE
            RAISE NOTICE 'Column last_accessed already exists.';
        END IF;
    END
$$;

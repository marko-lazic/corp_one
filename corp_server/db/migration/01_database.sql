CREATE TABLE players
(
    player_id  INTEGER PRIMARY KEY AUTOINCREMENT,
    username   VARCHAR(50) UNIQUE  NOT NULL,
    email      VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    level      INTEGER   DEFAULT 1,
    experience BIGINT    DEFAULT 0,
    coin       BIGINT    DEFAULT 0,
    last_login TIMESTAMP,

    UNIQUE (username),
    UNIQUE (email)
);

CREATE TABLE inventory
(
    inventory_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id      INTEGER     NOT NULL,
    inventory_type VARCHAR(50) NOT NULL,
    capacity       INTEGER,
    created_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (player_id) REFERENCES players (player_id) ON DELETE CASCADE
);

CREATE INDEX idx_inventory_player_id ON inventory (player_id);

CREATE TABLE items
(
    item_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    inventory_id INTEGER     NOT NULL,
    item_type    VARCHAR(50) NOT NULL,
    quantity     INTEGER   DEFAULT 1,
    attributes   TEXT,
    acquired_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_equipped  BOOLEAN   DEFAULT FALSE,
    durability   INTEGER,
    location     VARCHAR(50),

    FOREIGN KEY (inventory_id) REFERENCES inventory (inventory_id) ON DELETE CASCADE
);

CREATE INDEX idx_items_inventory_id ON items (inventory_id);
CREATE INDEX idx_items_item_type ON items (item_type);

-- Insert players
INSERT INTO players (username, email, level, experience, coin, last_login)
VALUES ('OmniSoldier', 'omni_soldier@fom.com', 15, 12000, 2500, '2024-11-01 14:00:00'),
       ('JupiterKnight', 'jupiter_knight@fom.com', 12, 9500, 1750, '2024-11-02 18:45:00'),
       ('ChronoAgent', 'chrono_agent@fom.com', 18, 16000, 3200, '2024-11-03 20:30:00'),
       ('VoidWanderer', 'void_wanderer@fom.com', 10, 7500, 1450, '2024-11-04 11:15:00'),
       ('SpectreSeeker', 'spectre_seeker@fom.com', 8, 6000, 1300, '2024-11-05 09:00:00');

-- Insert inventories for each player
INSERT INTO inventory (player_id, inventory_type, capacity)
VALUES (1, 'backpack', 30),
       (1, 'storage', 100),
       (2, 'backpack', 20),
       (2, 'loot_bag', 10),
       (3, 'storage', 150),
       (4, 'backpack', 25),
       (5, 'loot_bag', 15);

-- Insert items into inventories
INSERT INTO items (inventory_id, item_type, quantity, attributes, durability, location)
VALUES
-- Player 1 items
(1, 'laser_rifle', 1, '{"damage": 50, "range": 500, "energy_consumption": 10}', 80, 'backpack'),
(1, 'med_kit', 3, '{"healing": 20}', NULL, 'backpack'),
(2, 'grenade', 5, '{"damage": 35, "radius": 5}', NULL, 'storage'),
(2, 'nanobot_repair_tool', 1, '{"repair": 15}', 100, 'storage'),

-- Player 2 items
(3, 'plasma_pistol', 1, '{"damage": 30, "range": 300, "energy_consumption": 5}', 70, 'backpack'),
(3, 'field_rations', 10, '{"nutrition": 15}', NULL, 'backpack'),
(4, 'stealth_cloak', 1, '{"duration": 180}', 60, 'loot_bag'),

-- Player 3 items
(5, 'assault_rifle', 1, '{"damage": 40, "range": 400, "energy_consumption": 8}', 90, 'storage'),
(5, 'ammo_pack', 50, '{"caliber": "5.56mm"}', NULL, 'storage'),
(5, 'energy_cell', 10, '{"energy": 50}', NULL, 'storage'),

-- Player 4 items
(6, 'shock_gloves', 1, '{"damage": 15, "stun_duration": 3}', 85, 'backpack'),
(6, 'first_aid_kit', 2, '{"healing": 25}', NULL, 'backpack'),

-- Player 5 items
(7, 'combat_knife', 1, '{"damage": 10, "durability": 100}', 100, 'loot_bag'),
(7, 'bandages', 5, '{"healing": 10}', NULL, 'loot_bag');
-- Seed demo players for existing corp_login users
INSERT OR IGNORE INTO players (username, character_name, current_health, credits, faction, rank) VALUES
    ('commander_shepard', 'Commander Shepard', 100, 5000, 'EC', 'R3'),
    ('neo_anderson', 'Neo Anderson', 100, 2500, 'CMG', 'R2'),
    ('ellen_ripley', 'Ellen Ripley', 100, 3000, 'VI', 'R2'),
    ('duke_atreides', 'Paul Atreides', 100, 10000, 'EC', 'R4'),
    ('sarah_connor', 'Sarah Connor', 100, 1500, 'CMG', 'R1');
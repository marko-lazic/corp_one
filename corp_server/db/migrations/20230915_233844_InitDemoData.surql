-- ------------------------------
-- TRANSACTION
-- ------------------------------

BEGIN TRANSACTION;

-- ------------------------------
-- TABLE DATA: faction
-- ------------------------------

UPDATE faction:ec SET name = 'Euro Corp';
UPDATE faction:cmg SET name = 'Colonization and Mining Group';
UPDATE faction:vi SET name = 'Vort Inc';

-- ------------------------------
-- TABLE DATA: user
-- ------------------------------

UPDATE user:admin SET username = 'admin', email = 'admin@admin.rs', password = 'admin', registered_at = time::now();

RELATE user:admin->user_faction->faction:ec;

-- ------------------------------
-- TABLE DATA: colony
-- ------------------------------

UPDATE colony:liberte SET name = 'Liberte';
UPDATE colony:cloning SET name = 'Cloning';
UPDATE colony:iris SET name = 'Iris';

RELATE colony:liberte->colony_owned_by->faction:ec;
RELATE colony:iris->colony_owned_by->faction:vi;

-- ------------------------------
-- TRANSACTION
-- ------------------------------

COMMIT TRANSACTION;

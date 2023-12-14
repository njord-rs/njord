-- This file was automatically created by Njord to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

DROP FUNCTION IF EXISTS njord_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS njord_set_updated_at();
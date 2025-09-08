"""update tasks table user id

Revision ID: 28d6bdac1220
Revises: 934dc5d545dc
Create Date: 2025-09-08 11:53:07.503141

"""
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision: str = '28d6bdac1220'
down_revision: Union[str, Sequence[str], None] = '934dc5d545dc'
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    op.add_column('tasks', sa.Column('user_id', sa.Integer))


def downgrade() -> None:
    """Downgrade schema."""
    pass

-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。

CREATE INDEX idx_edges_node_a_id ON edges(node_a_id);
CREATE INDEX idx_nodes_area_id ON nodes(area_id);
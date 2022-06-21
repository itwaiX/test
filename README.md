# SeaQL TiDB - TiDB Support in the SeaQL ecosystem

Deliverables:
1. Replicate functionalities for TiDB in SeaQuery, SeaSchema and SeaORM. This should be made independent of MySQL. i.e. we might want to convert MySQL impl to a `MySqlLike` trait and then allow TiDB as a concrete impl to override some of the existing behaviour

3. Sample project and test suite that demonstrate use of TiDB
4. Tools for cluster mangement programmatically

Note: For now, this repo will clone SeaQuery, SeaSchema and SeaORM and develop a under a monorepo fashion. However at the end, we would like to merge the pieces back to the mother repo

Question: can we practically setup a CI for TiDB? (Will it be slow / use up a lot of our quota?) I definitely hope that TiDB can sponsor dedicated machines for testing

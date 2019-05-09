# Splitting a Mesh by NURBs grid (Maya)

1. Create NURBS shape or grid. This will work as a ‘cutter’ for the geometry in question.
2. Rotate the NURBS so that it is parallel with the geometry, and switch the topographical view ‘from above’.
3. Select the NURBS, and then shift-select the geometry. `Edit Mesh -> Project Curve on Mesh`
4. The curve projections will be selected by default. Shift-select the geometry as well. `Edit Mesh->Split Mesh with Projected Curve`
5. Plain select the geometry. `Mesh->Detach`, and then `Mesh->Separate`? Or maybe the other way around.
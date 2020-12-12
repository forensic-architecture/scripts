# Game Engine -> Annotated Datasets 

## TODO
- [ ] generalize `AnnotatedDataset` functions into trait, implemented by
    `SlyDataset`
- [ ] implement `too_tiny` function (needs to work from bbox rather than pixel
    #)
- [ ] correct colours for bitmap masks (they need to be a sort of grayscale)
- [ ] test suite to confirm masks are correct
- [ ] parallelise `Anns` construction, and then also dataset rendering
- [ ] implement `PascalVOCDataset` based on Python scripts


# Release Notes (1.0.0)
Breaking changes to most plot input types. Changed to references instead of ownership, and rewrote much underlying code.

## Release Notes (1.1.0)
For all plots, I added the ability to export them into a text file or as an image.

Updated how many internal structures were represented for more consistency. E.g. .build() on builder structs now does not mutate the underlying struct. I also made a few more helper functions public, even though they don't need to be. When I'm working on a math project, I often end up rewriting many of the helper functions used in this crate, so I'm doing this mainly for my own usecases, though some others may find them helpful.

### Release Notes (1.1.2)
Removed a forgotten print statement used in debugging
Implemented derive(Clone) on all plots

### Release Notes (1.1.3)
Added support for creating animations from custom image files, such as one may get from the new image export feature from 1.1.0

### Release Notes (1.1.6)
Omitted versions were simple bugfixes. 1.1.6 reduces the version back to 2021 for backwards compatability, since I accidentally broke SemVer. 1.1.6 also replaces bytemuck::Pod requirement for array plot floats with a better hashing algorithm based on the bytes of the floats. So bytemuck is now removed as a dependency

### Release Notes (1.1.7)
Added parallelization widely to drastically improve performance, especially for animation plot and image plot


## Release Notes (1.2.0)
Added documentation to all modules, helper files, and plots. This took a really long time, but my code is finally well-documented enough to be useful to others.

Also changed some small semantics for set_path methods on some plots.
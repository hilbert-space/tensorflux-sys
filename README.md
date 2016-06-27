# tensorflow-sys [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides bindings to [TensorFlow][tensorflow].

## [Documentation][documentation]

## [Example][example]

## Requirements

The build prerequisites of TensorFlow can be found on the [corresponding
page][setup] of TensorFlow’s documentation. In particular, [Bazel][bazel],
[NumPy][numpy], and [SWIG][swig] are assumed to be installed.

## Configuration

The compilation process of TensorFlow is configured via a number of environment
variables, all of which can be found in TensorFlow’s [configure][configure]
script. In particular, `TF_NEED_CUDA` is used to indicate if GPU support is
needed.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[bazel]: http://www.bazel.io
[configure]: https://github.com/tensorflow/tensorflow/blob/master/configure
[example]: examples/workflow.rs
[numpy]: http://www.numpy.org
[setup]: https://www.tensorflow.org/versions/r0.9/get_started/os_setup.html
[swig]: http://www.swig.org
[tensorflow]: https://www.tensorflow.org

[documentation]: https://stainless-steel.github.io/tensorflow-sys
[status-img]: https://travis-ci.org/stainless-steel/tensorflow-sys.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/tensorflow-sys
[version-img]: https://img.shields.io/crates/v/tensorflow-sys.svg
[version-url]: https://crates.io/crates/tensorflow-sys

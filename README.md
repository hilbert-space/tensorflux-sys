# tensorflow-sys [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides bindings to [TensorFlow][homepage].

## [Documentation][doc]

## Requirements

The build prerequisites of TensorFlow can be found on the [corresponding
page][setup] of TensorFlow’s documentation. In particular, the following
components are assumed to be installed:

* [Bazel](http://www.bazel.io) and
* [NumPy](http://www.numpy.org).

## Compilation

The configuration of the compilation process of TensorFlow is achieved via a
number of environment variables, which can be found in TensorFlow’s
[configure][configure] script. In particular,

* `PYTHON_BIN_PATH` is used to specify the location of Python, and
* `TF_NEED_CUDA` is used to indicate if GPU support is needed.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[configure]: https://github.com/tensorflow/tensorflow/blob/master/configure
[homepage]: https://www.tensorflow.org
[setup]: https://www.tensorflow.org/versions/r0.9/get_started/os_setup.html

[doc]: https://stainless-steel.github.io/tensorflow-sys
[status-img]: https://travis-ci.org/stainless-steel/tensorflow-sys.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/tensorflow-sys
[version-img]: https://img.shields.io/crates/v/tensorflow-sys.svg
[version-url]: https://crates.io/crates/tensorflow-sys

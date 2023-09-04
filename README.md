# Rust Version Control
This project is a version management tool developed in the Rust language. The project can be used to increment the version of a project, Git tag and commit changes automatically.

## Start
You can follow the steps below to run and develop the project on your local machine.

### Requirements
The following software must be installed to run this project:


- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads)

### Setup
#### Clone the project:

   ```sh
   git clone https://github.com/FurkanSamaraz/version_control.git
   ```

#### Go to the project folder:
```sh
cd version_control
```
#### Build the project:
```sh
cargo build
```

### Use
You can use the following commands to run the project:

#### To increase the version:

```sh
cargo run patch
```

#### For a minor version upgrade:

```sh
cargo run minor
```
#### For a major version upgrade:

```sh
cargo run major
```

## First Time using CMake
Following this guide: https://cliutils.gitlab.io/modern-cmake/chapters/intro/running.html

CMakeLists.txt is located in the root of project.  I then navigate to ./build and from there I run `cmake ..`
  - Why do I have to do this?
    - This seems to create all the platform specifc (e.g. msbuild) files in ./build

Then, also from ./build, i run `cmake --build .` which actually builds the project.
  - The `.` seems to refer to where the msbuild specific files are located



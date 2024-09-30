idf_component_register(SRCS "main.cpp"
                       INCLUDE_DIRS "."
                       PRIV_REQUIRES "wifi_manager")

set(MY_FLAGS
    "-Wall"
    "-Wextra"
    "-Wpedantic"
    "-Werror"
    "-Werror=unused-function"
    "-Werror=unused-variable"
    "-Werror=unused-but-set-variable"
    "-Werror=deprecated-declarations"
)

target_compile_options(${COMPONENT_LIB} PRIVATE ${MY_FLAGS})

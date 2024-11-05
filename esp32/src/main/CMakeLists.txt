idf_component_register(SRCS "src/main.cpp"
                       INCLUDE_DIRS "include"
                       PRIV_REQUIRES "esp-idf-wifi-manager"
                                     "esp_driver_gpio")

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

idf_component_register(SRCS "wifi_manager.cpp"
                       INCLUDE_DIRS "include"
                       EMBED_FILES assets/config_page.html
                       REQUIRES "esp_http_server"
                       PRIV_REQUIRES "esp_wifi"
                                     "log"
                                     "lwip")

set(MY_FLAGS
    "-Wall"
    "-Wextra"
    "-Werror"
    "-Werror=unused-function"
    "-Werror=unused-variable"
    "-Werror=unused-but-set-variable"
    "-Werror=deprecated-declarations"
)

target_compile_options(${COMPONENT_LIB} PRIVATE ${MY_FLAGS})

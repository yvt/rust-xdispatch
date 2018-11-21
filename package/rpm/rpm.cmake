

# set( CPACK_RPM_PACKAGE_NAME "xdispatch")
set( CPACK_RPM_PACKAGE_VERSION "${XDISPATCH_VERSION}.${PACKAGE_COMPILER}")
set( CPACK_RPM_PACKAGE_LICENSE "${XDISPATCH_LICENSE}")
set( CPACK_RPM_PACKAGE_REQUIRES "glibc")
set( CPACK_RPM_PACKAGE_VENDOR "mlba-team.de" )
if(MZ_IS_64BIT)
    set( CPACK_RPM_PACKAGE_ARCHITECTURE "x86_64")
else()
    set( CPACK_RPM_PACKAGE_ARCHITECTURE "x86")
endif()
set( CPACK_RPM_COMPONENT_INSTALL ON)

message("-- Configured generator 'RPM'")
set( XDISPATCH_CPACK ${XDISPATCH_CPACK} RPM )

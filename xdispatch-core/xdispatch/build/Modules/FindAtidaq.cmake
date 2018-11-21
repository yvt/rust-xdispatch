FIND_PATH(
	ATIDAQ_INCLUDE_DIRS
	NAMES
	atidaq/ftconfig.h
	HINTS
	$ENV{HOME}/include
	/usr/local/include
	/usr/include
)

FIND_LIBRARY(
	ATIDAQ_LIBRARIES
	NAMES
	atidaq
	HINTS
	$ENV{HOME}/lib
	/usr/local/lib
	/usr/lib
)

FIND_PACKAGE_HANDLE_STANDARD_ARGS(
	ATIDAQ
	DEFAULT_MSG
	ATIDAQ_INCLUDE_DIRS
	ATIDAQ_LIBRARIES
)

MARK_AS_ADVANCED(
	ATIDAQ_INCLUDE_DIRS
	ATIDAQ_LIBRARIES
) 

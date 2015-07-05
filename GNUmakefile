VERSION='0.0.1'

PKG_CONFIG_LIBS = xcb xcb-xkb xcb-atom xkbcommon xkbcommon-x11

.DEFAULT_GOAL = perminal

#
# sources
# 

SRC=main.cc 			\
    emulator/cursor.cc		\
    emulator/debug.cc		\
    emulator/charencoding.cc	\
    emulator/matrix.cc		\
    emulator/dumbterminal.cc	\
    terminal/pty.cc		\
    renderer/xcbrenderer.cc	\
    renderer/xkb_keyboard.cc	\
    renderer/font.cc		\
    renderer/bitmapfont.cc

#
# compilation options
#

# installation directory
PREFIX=/usr/local

# check for libraries
CPPFLAGS += `pkg-config --cflags ${PKG_CONFIG_LIBS}` 
LDFLAGS += `pkg-config --libs ${PKG_CONFIG_LIBS}` -lutil

# header directory
CPPFLAGS += -Iemulator -Iterminal -Irenderer -isystem renderer/system

# default compilation options
CPPFLAGS += -fdiagnostics-color=auto -pipe -std=c++1y -DVERSION=\"${VERSION}\" -DDATADIR=\"../data\" -fPIC -MMD -MP

# add debugging
ifeq (${DEBUG},1)
  CPPFLAGS += -g3 -ggdb -DDEBUG
endif

# add warnings
ifeq (${WARNINGS},1)
  CXXFLAGS += \
      -Wextra  -Wall -Wcast-align -Wcast-qual  -Wchar-subscripts  -Wcomment \
      -Wdisabled-optimization -Wfloat-equal  -Wformat  -Wformat=2 \
      -Wformat-nonliteral -Wformat-security -Wformat-y2k -Wimport  -Winit-self \
      -Winvalid-pch -Wmissing-braces -Wmissing-field-initializers -Wmissing-format-attribute   \
      -Wmissing-include-dirs -Wmissing-noreturn -Wpacked -Wparentheses  -Wpointer-arith \
      -Wredundant-decls -Wreturn-type -Wsequence-point  -Wsign-compare  -Wstack-protector \
      -Wstrict-aliasing -Wstrict-aliasing=2 -Wswitch -Wtrigraphs  -Wuninitialized \
      -Wunknown-pragmas  -Wunreachable-code -Wunused -Wunused-function  -Wunused-label \
      -Wunused-parameter -Wunused-value  -Wunused-variable  -Wvariadic-macros \
      -Wvolatile-register-var  -Wwrite-strings -Wfatal-errors -Winvalid-pch -Weffc++ \
      -Wold-style-cast -Wsign-promo -Wswitch-enum -Wmissing-declarations -Wfatal-errors
  ifeq (${CXX},g++)
    CXXFLAGS += -Wunsafe-loop-optimizations -Wzero-as-null-pointer-constant -Wuseless-cast
  endif
endif
CXXFLAGS += -Wno-narrowing

# add optimization
ifneq (${NO_OPT},1)
  CPPFLAGS += -Ofast -fomit-frame-pointer -ffast-math -mfpmath=sse -fPIC -msse -msse2 -msse3 -mssse3 -msse4
  CPPFLAGS += -Wno-inline
endif

# debug Makefile
Q := @
ifeq (${DEBUG_MAKE},1)
  Q =
endif


#
# constants
#

red =\033[0;31m
green =\033[0;32m
magenta =\033[0;35m
done =\033[0m

LINT_FILTERS = -legal,-build/include,-whitespace,-readability/namespace,-readability/function,-build/namespaces,-readability/todo,-build/c++11,-runtime/references,-build/header_guard

SUPPRESSIONS=xcb.supp


# 
# print Makefile information

$(info DEBUG    = $(if ${DEBUG},yes,no))
$(info NO_OPT   = $(if ${NO_OPT},yes,no))
$(info WARNINGS = $(if ${WARNINGS},yes,no))
$(info CXX      = ${CXX})
$(info LDLFLAGS = ${LDFLAGS})
$(info CPPFLAGS = ${CPPFLAGS} $(if $WARNINGS,...WARNINGS...))
$(info ----------------)


#
# compile C++ sources
#

OBJ = ${SRC:.cc=.o}

.cc.o: 
ifneq (${DEBUG_MAKE},1)	
	@echo ${CXX} -c $<
endif
	${Q} ${CXX} -c ${CPPFLAGS} ${CXXFLAGS} $< -o $@

# 
# compile font image
#
main.cc: renderer/latin1.xbm

renderer/latin1.xbm: renderer/latin1.png
	@echo CONVERT $@
	@convert $< $@
	@sed -i 's/static char/static unsigned char/g' renderer/latin1.xbm

#
# rules
#

perminal: checklibs ${OBJ}
ifneq (${DEBUG_MAKE},1)	
	@echo -e '${green}${CXX} -o $@${done}'
endif
	${Q} ${CXX} -o $@ $(filter-out $<,$^) ${EXTRA_LIBS} ${CPPFLAGS} ${CXXFLAGS} ${LDFLAGS}


test:
	${MAKE} DEBUG=1 NO_OPT=1 WARNINGS=1
	./perminal

#
# non-compilation rules
# 
clean:
ifneq (${DEBUG_MAKE},1)	
	@echo -e '${red}cleaning${done}'
endif
	${Q} rm -f perminal *.o *.d **/*.o **/*.d

gensuppressions: perminal
ifneq (${DEBUG_MAKE},1)	
	@echo -e '${red}creating suppression list${done}'
endif
	@valgrind --leak-check=full --show-leak-kinds=all --gen-suppressions=yes \
	  $(addprefix --suppressions=util/,${SUPPRESSIONS}) ./perminal

checkleaks: perminal
ifneq (${DEBUG_MAKE},1)	
	@echo -e '${red}checking for memory leaks${done}'
endif
	@valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes \
	  $(addprefix --suppressions=util/,${SUPPRESSIONS}) ./perminal

countlines:
	@cloc --exclude_dir=system,.rust,docs --force-lang="C++",h .

lint: 
	cpplint --filter=${LINT_FILTERS} --linelength=120 **/*.cc **/*.h

help:
	@echo 'Variables that will influence this make:'
	@echo '  CXX          choose a different compiler'
	@echo '  NO_OPT       turn off all optimizations'
	@echo '  WARNINGS     turn on all warnings'
	@echo '  DEBUG        create debug symbols'
	@echo '  DEBUG_MAKE   debug this Makefile'

install: perminal
	@echo INSTALL perminal
	@install perminal ${PREFIX}/bin

uninstall: perminal
	@echo RM perminal
	@rm -f ${PREFIX}/bin/perminal
	
dist:
	${MAKE} clean
	mkdir perminal-${VERSION}
	cp -R GNUmakefile README INSTALL TODO main.cc perminal-${VERSION}
	cp -R emulator terminal renderer perminal-${VERSION}
	tar cvf perminal-${VERSION}.tar.gz perminal-${VERSION}
	rm -rf perminal-${VERSION}

checklibs:
	@for lib in ${PKG_CONFIG_LIBS}; do \
	  pkg-config $$lib && ([ $$? -eq 0 ]) || { echo -e "${red}Library '$$lib' was not found by pkg-config. This library is required for building 'perminal'.${done}"; exit 1; } \
	done
	@#echo -e "${green}Required libraries installed.${done}"

.PHONY: all clean help checkleaks lint gensuppressions install uninstall dist checklibs

-include ${SRC:.cc=.d}

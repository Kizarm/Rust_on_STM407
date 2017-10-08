EXES = ast

CXX := clang++-3.8
#RTTIFLAG := -fno-rtti
#CXXFLAGS := $(shell llvm-config-3.8 --cxxflags) $(RTTIFLAG)
# trochu upravime - je tam warning souvisici s -Wno-uninitialized
CXXFLAGS := -I/usr/lib/llvm-3.8/include -std=c++0x -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual \
 -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -std=c++11 \
 -ffunction-sections -fdata-sections -O2 -g -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS \
 -D__STDC_LIMIT_MACROS

LLVMLDFLAGS := $(shell llvm-config-3.8 --ldflags --libs --system-libs)

OBJECTS = ast.o

# z webu, musi to mit spravne poradi
CLANGLIBS =-lclangTooling -lclangFrontendTool -lclangFrontend -lclangDriver -lclangSerialization -lclangCodeGen -lclangParse -lclangSema \
 -lclangStaticAnalyzerFrontend -lclangStaticAnalyzerCheckers -lclangStaticAnalyzerCore -lclangAnalysis -lclangARCMigrate \
 -lclangRewrite -lclangRewriteFrontend -lclangEdit -lclangAST -lclangLex -lclangBasic -lclang

all: $(EXES)
.PHONY: clean

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c -o $@ $<

$(EXES): $(OBJECTS)
	$(CXX) -o $(EXES) $(OBJECTS) $(CLANGLIBS) $(LLVMLDFLAGS)

clean:
	-rm -f $(OBJECTS) *~
	-rm -f $(EXES)


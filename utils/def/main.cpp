#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void parse (const char * iname, const char * oname) {
  size_t max = 1024;
  FILE * in   = fopen (iname, "r");
  FILE * out  = fopen (oname, "w");
  char * line = new char [max];
  int result;
  
  fprintf (out, "#include \"stm32f4xx.h\"\n");
  for (;;) {
    result = getline (&line, &max, in);
    if (result <= 0) break;
    const unsigned len = 256;
    char name [len], type [len], base [len];
    int n = sscanf (line, "#define %s ((%s *) %s)", name, type, base);
    if (n==3) {
      fprintf (out, "static %s * const %s = (%s * const) (%s;\n", type, name, type, base);
    }
  }

  delete [] line;
  fclose (in);
  fclose (out);
}

int main (void) {
  parse ("def.h", "def.c");
  return 0;
}

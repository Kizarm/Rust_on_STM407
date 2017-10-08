#include "clang/AST/ASTConsumer.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendAction.h"
#include "clang/Tooling/Tooling.h"
#include "clang/AST/Comment.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Decl.h"
#include "clang/AST/CommentVisitor.h"
#include "clang/AST/StmtVisitor.h"
#include "clang/AST/DeclVisitor.h"

#include <iostream>
#include <fstream>

#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

static void fixw (std::__cxx11::string & in, unsigned w) {
  unsigned l = in.length();
  if (w > l) {
    unsigned m = w - l;
    for (unsigned n=0; n<m; n++) in += " ";
  }
}
struct RSTypeDef {
  bool                    pub;        // vzdy true
  bool                    hidden;
  bool                    simple;
  bool                    used;
  unsigned long           array;      // 0 - jednoducha promenna
  unsigned                width;
  std::__cxx11::string    name;
  std::__cxx11::string    type;
  std::__cxx11::string    comment;
  std::vector<RSTypeDef>  elem;
  RSTypeDef             * kind;
  void clear (void) {
    pub    = false;
    hidden = false;
    simple = false;
    used   = false;
    array = 0ul;
    width = 0u;
    name.clear();
    type.clear();
    elem.clear();
    comment.clear();
    kind = nullptr;
  }
  void add (RSTypeDef &f) {
    elem.push_back  (f);
  }
  void dump_part (std::ostream & os) const {
    std::__cxx11::string tcpy = type;
    if (array) tcpy = "[" + tcpy + "; " + std::to_string(array) + "]";
    if (array) tcpy = " :"  + tcpy;
    else       tcpy = " : " + tcpy;
    tcpy += ",";
    std::__cxx11::string dmp = "  ";
    std::__cxx11::string fixn = name;
    std::__cxx11::string fixt = tcpy;
    if (pub) dmp += "pub ";
    else     dmp += "    ";
    fixw (fixn, 10);
    dmp += fixn;
    if (!comment.empty()) {
      fixw (fixt, 22);
      dmp += fixt;
      dmp += "/*" + comment + "*/";
    } else {
      dmp += fixt;
    }
//  dmp += "/* " + std::to_string (width) + " */\n";
    dmp += "\n";
    
    os << dmp;
  }
  void dump (std::ostream & os) const {
    if (hidden) return;
    if (used) printf ("  USED: %s\n", name.c_str());
    const unsigned len = 64;
    char buf [len];
    snprintf (buf, len, "/* 0x%04X */", width);
    if (width & 3) {
      fprintf (stderr, "Tohle je divne %04X (%s)\n", width, name.c_str());
    }
    if (elem.empty()) {
      // dump_part(os); // zbytecne
    } else {
      std::__cxx11::string dmp;
      dmp = "\n#[repr(C, packed)]\n";
      if (used) dmp += "#[derive(Copy, Clone)]\n";
      if (pub)  dmp += "pub ";
      dmp += "struct " + name + " {\n";
      os << dmp;
      for (auto n: elem) n.dump_part(os);
      os << "}" << buf <<  "\n";
    }
  }
  void reference (std::vector<RSTypeDef> & d);
  void initialize (std::__cxx11::string& str, const unsigned ind = 2);
};
void RSTypeDef::initialize (std::__cxx11::string & str, const unsigned ind) {
  char * buf = new char [ind + 1];
  unsigned cnt;
  for (cnt=0; cnt<ind; cnt++) buf[cnt] = ' ';
  buf [cnt] = '\0';
  std::__cxx11::string indent = buf;
  delete [] buf;
  
  //if (used)
  //  std::cout << "T:Enter " << name << " [" << type << "]" << "\n";
  if (elem.empty()) {
    if (name == "0") str += "0,\n";
    else             str += indent + name + " : ";
    if (array) str += "[";
  } else {
    str += name   +  " {\n";
    for (RSTypeDef & e: elem) e.initialize(str, ind + 2);
  }
  if (kind) {
    //printf ("  %s:KIND:%s %p => %p\n", name.c_str(), kind->name.c_str(), (void*) this, (void*) kind);
    kind->initialize(str, ind + 2);
  }
  if (elem.empty()) {
    if (array) {
      unsigned n = str.find_last_of(",");
      if (n > 0) {
        unsigned m = str.length() - n;
        str.erase (n,m);
      }
      const unsigned len = 64;
      char tmp [len];
      snprintf (tmp, len, "; 0x%04lX],\n", array);
      str += tmp;
    }
  } else {
    str += indent + " },\n";
  }
  //str += ",\n";
}

void RSTypeDef::reference (std::vector< RSTypeDef >& d) {
  //printf ("entry: %s(t=%s), w=%d\n", name.c_str(), type.c_str(), width);
  if (!elem.empty()) {
    unsigned w = 0;
    for (RSTypeDef & n: elem) {
      n.reference (d);
      w += n.width;
      width = w;
    }
    return;
  }
  for (RSTypeDef & e: d) {
    if (name == "0") {
      return;
    }
    //printf ("  compare : %s <=> %s\n", type.c_str(), e.type.c_str());
    if (type  == e.type) {
      //printf ("  - %s, w=%d=>%d", e.type.c_str(), e.width, width);
      if (array) width = e.width * array;
      else       width = e.width;
      //printf (" T: %d\n", width);
      if (!kind) {
        kind = & e;
        e.used = true;
        //printf ("  - kind %s=>%s\n", e.name.c_str(), name.c_str());
      }
      break;
    }
  }
  //printf ("end  : %s(t=%s), w=%d\n", name.c_str(), type.c_str(), width);
}

struct RSDescription {
  std::vector<RSTypeDef>  elem;
  void add (RSTypeDef & e) {
    elem.push_back(e);
  }
  void add (RSDescription & other) {
    for (auto e: other.elem) add (e);
  }
  void dump (std::ostream & os) const {
    for (auto n: elem) n.dump(os);
  }
  void reference (void) {
    for (RSTypeDef & e: elem) e.reference (elem);
  }
  void clear (void) {
    elem.clear();
  }
};
static void add_u (RSDescription & d, const unsigned n) {
  RSTypeDef nd;
  nd.clear();
  const unsigned len = 16;
  char buf [len];
  snprintf (buf, len, "u%d", n);
  nd.name = "0";
  nd.type = buf;
  nd.width = n >> 3;
  nd.hidden = true;
  nd.simple = true;
  d.add (nd);
}
static void add_s (RSDescription & d, RSTypeDef & e, const char * s) {
  RSTypeDef nd;
  RSTypeDef cp;
  cp = e;
  cp.name = "value";
  //cp.kind = & e;
  nd.clear();
  
  const unsigned len = 32;
  char buf [len];
  snprintf (buf, len, "%s<%s>", s, cp.type.c_str());
  nd.name = s;
  nd.type = buf;
  nd.add (cp);
  //nd.width  = e.width;
  nd.hidden = true;
  d.add (nd);
}
static void add_primitives (RSDescription & d) {
  unsigned n;
  unsigned tb [] = {8,16,32};
  
  for (n=0; n<3; n++) add_u (d,tb [n]);
  for (n=0; n<3; n++) add_s (d, d.elem.at(n), "RWReg");
  for (n=0; n<3; n++) add_s (d, d.elem.at(n), "ROReg");
}

struct RSLocation {
  bool                    pub;        // vzdy true;
  std::__cxx11::string    name;
  std::__cxx11::string    type;
  unsigned long           addr;
  
  RSTypeDef             * anot;
  unsigned                padd;
  unsigned                padno;
  void clear (void) {
    pub = true;
    name.clear();
    type.clear();
    addr = 0ul;
    
    anot  = nullptr;
    padd  = 0u;
    padno = 0u;
  }
  void dump (std::ostream & os) {
    std::__cxx11::string dmp;
    std::__cxx11::string tcp = "*const " + type;
    std::__cxx11::string ncp = name;
    std::__cxx11::string tmp = tcp;
    fixw (ncp, 14);
    fixw (tmp, 24);
    char buf [64];
    snprintf (buf, 64, " = 0x%08lXu32", addr);
    dmp = "pub const " + ncp + ": " + tmp + buf + " as " + tcp + ";\n";
    
    os << dmp;
  }
  void anotate (std::vector< RSTypeDef >& d);
  void build   (std::ostream & os);
  void initialize (std::__cxx11::string & str);
};
void RSLocation::initialize (std::__cxx11::string& str) {
  //std::cout << "Enter " << name << "\n";
  if (padd) {
    const unsigned len = 256;
    char buf [len];
    snprintf (buf, len, "  padding%02d: [0; 0x%04X],\n", padno, padd >> 2);
    str += buf;
  }
  std::__cxx11::string ncpy = name;
  std::transform (ncpy.begin(), ncpy.end(), ncpy.begin(), ::tolower);
  str += "  " + ncpy + " : ";
  if (anot) anot->initialize (str);
}

void RSLocation::anotate (std::vector< RSTypeDef >& d) {
  int n = type.find(" ");       // v CMSIS jsou v type mezery
  if (n > 0) {
    int m = type.length() - n;
    type.erase (n, m);
  }
  bool result = false;
  for (RSTypeDef & e: d) {
    if (type == e.type) {
      anot = &e;
      result = true;
      break;
    }
  }
  if (!result) {
    fprintf (stderr, "Not annotation for %s <%s>\n", name.c_str(), type.c_str());
  } else {
    // printf ("anotation %p to <%s>\n", (void *) anot, name.c_str());
  }
}
static unsigned long RSOffset = 0;
static unsigned long RSIndex  = 0;
static unsigned long OldSize  = 0;

void RSLocation::build (std::ostream& os) {
  padno = RSIndex++;
  if (anot != nullptr) {
    padd = addr - OldSize - RSOffset;
    RSOffset = addr;
    OldSize  = anot->width;
    std::__cxx11::string dmp;
    std::__cxx11::string ncpy = name;
    std::transform (ncpy.begin(), ncpy.end(), ncpy.begin(), ::tolower);
    fixw (ncpy, 8);
    if (padd) {
      const unsigned len = 128;
      char buf [len];
      snprintf (buf, len, "  padding%02d    : [u32; 0x%04X],%*s/*             w=<0x%04X> */\n", padno, padd >> 2, 3, " ", padd);
      dmp += buf;
    }
    const unsigned wa = 128;
    char ba [wa];
    int wt = type.length();
    const int mt = 16;
    if (mt > wt) wt = mt - wt;
    else         wt = 0;
    snprintf (ba, wa, ",%*s/* 0x%08lX, w=<0x%04X> */\n", wt, " ", addr, anot->width);
    dmp += "  pub " + ncpy + " : " + type + ba;
    os << dmp;
  } else {
    fprintf (stderr, "chybi anotace at %d <%s>\n", padno, name.c_str());
    OldSize = 0ul;
  }
  //printf ("%08lX<%s>[w=%04lX] pad:%04X\n", addr, name.c_str(), OldSize, padd);
}

static bool compare (const RSLocation & a, const RSLocation & b) {
  return a.addr < b.addr;
}
struct RSDevice {
  std::vector<RSLocation> elem;
  
  void add (RSLocation & e) {
    elem.push_back(e);
  }
  void dump (std::ostream & os) {
    for (RSLocation & n: elem) n.dump(os);
  }
  void sort (void) {
    std::sort (elem.begin(), elem.end(), compare);
  }
  void anotate (std::vector< RSTypeDef >& d) {
    for (RSLocation & n: elem) n.anotate (d);
  }
  void build (std::ostream& os) {
    RSOffset = 0x40000000ul;
    RSIndex  = 0ul;
    OldSize  = 0ul;
    os << "#[repr(C, packed)]\n";
    os << "pub struct STM32F4Device {\n";
    for (RSLocation & n: elem) {
      if (n.addr <  0x50000000) n.build (os);
    }
    os << "}\n";
    for (RSLocation & n: elem) {
      if (n.addr >= 0x50000000) n.dump  (os);
    }
  }
  void initialize (std::__cxx11::string & str) {
    str += "\n#[link_section=\".device_reg\"]  /* hodne divny hack */\n";
    str += "pub static STM : STM32F4Device = STM32F4Device {\n";
    for (RSLocation & n: elem) {
      if (n.addr <  0x50000000) {
        n.initialize (str);
      }
    }
    str += "};\n";
  }
};
static RSDescription RSStructs;
static RSDevice      STM32;

using namespace clang;

class PartDumper :
  public comments::ConstCommentVisitor<PartDumper>,
  public ConstStmtVisitor<PartDumper>,
  public ConstDeclVisitor<PartDumper>  {
  public:
    RSTypeDef  RSF;
    RSTypeDef  RST;
    RSLocation RSL;

  public:
    explicit PartDumper () {
      RSF.clear();
      RST.clear();
      RSL.clear();
    }
    // komentáře mají rekurzivní strukturu, parsuje se takto divně
    template<typename Fn> void dumpChild (Fn doDumpChild) {
      doDumpChild();
    }
    void dumpComment (const comments::Comment *C) {
      dumpChild ([=] {
        if (!C) {
          return;
        }
        comments::ConstCommentVisitor<PartDumper>::visit (C);
        for (comments::Comment::child_iterator I = C->child_begin(),
                                               E = C->child_end();
             I != E; ++I) dumpComment (*I);
      });
    }
    // tohle vyplivne vlastní text komentáře
    void visitTextComment (const comments::TextComment *C) {
      std::__cxx11::string str = C->getText();
      RSF.comment += str;
    }
    void evalStmt (const Stmt *S) {
      dumpStmt (S);
      RSL.addr = address;
      address  = 0;
    }
    void dumpStmt (const Stmt *S) {
      dumpChild ([=] {
        if (!S) {
          return;
        }
        if (const DeclStmt *DS = dyn_cast<DeclStmt> (S)) {
          VisitDeclStmt (DS);
          return;
        }
        ConstStmtVisitor<PartDumper>::Visit (S);
        for (const Stmt *SubStmt : S->children())
          dumpStmt (SubStmt);
      });
    }
    void dumpDecl (const Decl *D) {
      dumpChild ([=] {
        if (!D) {
          return;
        }
        ConstDeclVisitor<PartDumper>::Visit (D);
      });
    }

    void VisitStmt (const Stmt * Node) {
      // llvm::outs() << Node->getStmtClassName() << "\n";
    }

    void VisitDeclStmt (const DeclStmt *Node) {
      VisitStmt (Node);
      for (DeclStmt::const_decl_iterator I = Node->decl_begin(),
                                         E = Node->decl_end();
           I != E; ++I) dumpDecl (*I);
    }
    void VisitIntegerLiteral(const IntegerLiteral *Node) {
      bool isSigned = Node->getType()->isSignedIntegerType();
      std::__cxx11::string str = Node->getValue().toString(10, isSigned);
      unsigned long val = std::strtoul(str.c_str(), 0, 10);
      address += val;
    }
  private:
    unsigned long address;
};

class FindNamedClassVisitor :
  public RecursiveASTVisitor<FindNamedClassVisitor> {
  public:
    explicit FindNamedClassVisitor (ASTContext *) : PD() {
    }
    bool VisitTypedefDecl (TypedefDecl * Dt) {
      const Type * t = Dt->getTypeForDecl();
      if (!t)                 {
        PD.RST.clear();
        return true;
      }
      if (!t->isRecordType()) {
        PD.RST.clear();
        return true;
      }
      
      std::__cxx11::string name = Dt->getNameAsString();
      int n = name.find("Def");
      if (n > 0) {
        int m = name.length() - n;
        name.erase(n, m);
      }
      PD.RST.name = name;
      PD.RST.type = name;
      PD.RST.pub  = true;
      RSStructs.add(PD.RST);
      PD.RST.clear();
      return true;
    }
    bool VisitRecordDecl (RecordDecl * Dr) {
      return true;
    }

    bool VisitFieldDecl (FieldDecl *Declaration) {
      PD.RSF.clear();
      QualType t = Declaration->getType();
      std::__cxx11::string name = Declaration->getName();
      std::__cxx11::string desc = t.getAsString();
      // std::__cxx11::string type = "/*EDIT!*/" + desc;
      std::__cxx11::string type = desc;
      int pos = type.find("Def");
      if (pos > 0) {
        int n = type.length() - pos;
        type.erase(pos, n);
      }
      std::transform (name.begin(), name.end(), name.begin(), ::tolower);
      //if (desc.find ("union")    != std::__cxx11::string::npos) type = "/*EDIT!*/u32";
      if (desc.find ("union")    != std::__cxx11::string::npos) type = "u32";
      if (desc.find ("uint32_t") != std::__cxx11::string::npos) type = "u32";
      if (desc.find ("uint16_t") != std::__cxx11::string::npos) type = "u16";
      if (desc.find ("uint8_t")  != std::__cxx11::string::npos) type = "u8";
      // nekde se v C pouziva oboji, zde nema vyznam, const ma prednost
      if      (desc.find ("const")    != std::__cxx11::string::npos) type = "ROReg<" + type + ">";
      else if (desc.find ("volatile") != std::__cxx11::string::npos) type = "RWReg<" + type + ">";
      pos = desc.find ("[");
      if (pos > 0) {
        int arr = 0, res = 0;
        const char * cdes = desc.c_str() + pos;
        res = sscanf (cdes, "[%d]", &arr);
        if (res == 1) {
          PD.RSF.array = arr;
        } else {
          PD.RSF.array = 0;
        }
      }
      if (name.find ("reserved") != std::__cxx11::string::npos) {
        PD.RSF.pub = false;
      } else {
        PD.RSF.pub = true;
      }
      PD.RSF.name = name;
      PD.RSF.type = type;
      /* Takhle z toho jdou vypatlat komentáře.
       * Inspirace je v <a href="http://clang.llvm.org/doxygen/ASTDumper_8cpp_source.html">dokumentaci</a>,
       * takhle by šlo parsovat ceký strom, ale nestojí to za tu práci. Je to moc složité.
       * */
      const comments::FullComment * mComment = Declaration->getASTContext().getLocalCommentForDeclUncached (Declaration);
      if (mComment) {
        PD.dumpComment (mComment);
      }
      PD.RST.add(PD.RSF);
      return true;
    }
    
    bool VisitVarDecl (VarDecl * Dv) {
      QualType t = Dv->getType();
      std::__cxx11::string type = t.getAsString();
      std::__cxx11::string name = Dv->getNameAsString();
      int n = type.find("Def");
      if (n > 0) {
        int m = type.length() - n;
        type.erase(n, m);
      }
      std::transform (name.begin(), name.end(), name.begin(), ::toupper);
      n = type.find ("*const");
      if (n > 0) {
        int m = type.length() - n;
        type.erase (n, m);
      }
      PD.RSL.type = type;
      PD.RSL.name = name;
      
      Expr * e = Dv->getInit();
      PD.evalStmt (e);
      STM32.add (PD.RSL);
      PD.RSL.clear();
      return true;
    }
    

  private:
    PartDumper PD;
};

class FindNamedClassConsumer : public clang::ASTConsumer {
  public:
    explicit FindNamedClassConsumer (ASTContext *Context)
      : Visitor (Context) {}

    virtual void HandleTranslationUnit (clang::ASTContext &Context) {
      Visitor.TraverseDecl (Context.getTranslationUnitDecl());
    }
  private:
    FindNamedClassVisitor Visitor;
};

class FindNamedClassAction : public clang::ASTFrontendAction {
  public:
    virtual std::unique_ptr<clang::ASTConsumer> CreateASTConsumer (
      clang::CompilerInstance &Compiler, llvm::StringRef InFile) {
      return std::unique_ptr<clang::ASTConsumer> (
               new FindNamedClassConsumer (&Compiler.getASTContext()));
    }
};

// ono to ma vevnitr nejake prostredky pro soubory, ale nez to nastuduji, bude tohle hotove
char * read_file (const char * name) {
  struct stat buf;
  int res = stat (name, &buf);
  if (res) return 0;
  char * context = new char [buf.st_size];
  FILE * in = fopen (name, "r");
  res = fread (context, 1, buf.st_size, in);
  fclose (in);
  // printf ("readen %ld bytes\n", buf.st_size);
  return context;
}
static RSDescription RSReferences;

int main (int argc, char **argv) {
  if (argc > 1) {
    std::__cxx11::string oname;
    if (argc > 2) {
      oname = argv[2];
    } else {
      oname = argv[1];
      int n = oname.find(".");
      if (n > 0) {
        int m = oname.length() - n;
        oname.erase (n, m);
      }
      oname += ".rs";
    }
    std::filebuf fb;
    fb.open(oname, std::ios::out);
    std::ostream os (&fb);
    os << "use vc::{RWReg,ROReg};\n";

    char * context = read_file (argv[1]);
    if (!context) return 1;
    llvm::Twine ctx = context;
    clang::tooling::runToolOnCode (new FindNamedClassAction, ctx, argv[1]);

    //RSStructs.dump(os);
    STM32.sort();
    //STM32    .dump(os);
    
    RSReferences.clear();
    add_primitives(RSReferences);
    RSReferences.add(RSStructs);
    
    RSReferences.reference();
    RSReferences.dump(os);
    
    STM32.anotate (RSReferences.elem);
    printf ("Anotation end\n");
    STM32.build(os);
    // STM32.dump (os);
    
    std::__cxx11::string init;
    STM32.initialize(init);
    //std::cout << init;
    os << init;
    
    fb.close();
  } else {
    printf ("Usage :\n\t%s infile [outfile]\n", argv[0]);
  }
  return 0;
}


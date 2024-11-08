# Sistema de Diretórios e Permissões em Rust
### Esse sistema consiste em algo parecido com Linux, implementando sistemas de arquivos, diretórios, usuários, grupos e permissões.

### Structs:
```Rust
#[derive(Clone)]
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    usuario: Usuario,
    grupo: Grupo,
}
#[derive(Clone)]
struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}
#[derive(Clone)]
struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    dono: Usuario,
}
#[derive(Clone)]
struct Usuario {
    nome: String,
    uid: u16,
    grupo: Grupo,
}
#[derive(Clone)]
struct Grupo {
    nome: String,
    gid: u16,
    membros: Vec<Usuario>,
}
```
### Impl:
```Rust
impl Arquivo{
    fn new(nome: String, tamanho: u64, usuario: Usuario, grupo: Grupo) -> Arquivo {
        Arquivo{
            nome,
            tamanho,
            permissoes: (Permissao {
                leitura: false,
                escrita: true,
                execucao: false,
            }, Permissao {
                leitura: false,
                escrita: true,
                execucao: false,
            }, Permissao {
                leitura: false,
                escrita: true,
                execucao: false,
            }),
            usuario,
            grupo,
        }

    }
    fn alterar_permissoes(&mut self, permi: (Permissao, Permissao, Permissao)){
        self.permissoes = permi;
    }
    fn stat(&self) {
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {}", self.tamanho);
        println!(
            "Permissões: {}/{}{}{}", format!("{}{}{}", self.permissoes.0.octal(), self.permissoes.1.octal(), self.permissoes.2.octal()),
            self.permissoes.0.rwx(), self.permissoes.1.rwx(), self.permissoes.2.rwx());
        println!("Uid: {}", self.usuario.uid);
        println!("Gid: {}", self.usuario.grupo.gid);
    }
}
impl Permissao {
    fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao {leitura, escrita, execucao}
    }
    fn octal(&self) -> u8 {
        let a: u8 = self.leitura as u8;
        let b: u8 = self.escrita as u8;
        let c: u8 = self.execucao as u8;

        match (a, b, c) {
            (0, 0, 0) => 0,
            (0, 0, 1) => 1,
            (0, 1, 0) => 2,
            (0, 1, 1) => 3,
            (1, 0, 0) => 4,
            (1, 0, 1) => 5,
            (1, 1, 0) => 6,
            (1, 1, 1) => 7,
            _ => 0,
        }
    }
    fn rwx(&self) -> String{
        let leitura = if self.leitura {'r'} else {'-'};
        let escrita = if self.escrita  {'w'} else {'-'};
        let execucao = if self.execucao  {'x'} else {'-'};

        format!("{}{}{}", leitura, escrita, execucao)
    }
}
impl Diretorio {
    fn new(nome: String, permissao: (Permissao, Permissao, Permissao), dono: Usuario) -> Diretorio {
        Diretorio{
            nome,
            arquivos: Vec::new(),
            permissoes: permissao,
            dono,
        }
    }
}
impl Usuario {
    fn new(nome: String, uid: u16, grupo: Grupo) -> Usuario {
        Usuario{nome, uid, grupo}
    }
    fn adiciona_grupo(&mut self, grupo: Grupo) {
        self.grupo = grupo;
    }

    fn listar_grupos(&self) {
        println!("Grupo: {}", self.grupo.nome);
    }
}
impl Grupo{
    fn new(nome: String, gid: u16) -> Grupo {
        Grupo{
            nome,
            gid,
            membros: Vec::new(),
        }
}
```

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

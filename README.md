# Sistema de Diretórios e Permissões em Rust
### Esse sistema consiste em algo parecido com Linux, implementando sistemas de arquivos, diretórios, usuários, grupos e permissões.

### Base para o projeto:
![image](https://github.com/user-attachments/assets/f1b08705-ce84-44cc-9aca-a21f4b179471)

![image](https://github.com/user-attachments/assets/a701f331-f5dc-4e99-8345-d3dab10937cb)

### Estrutura Permissao
```
• No método new você deverá atribuir aos campos leitura, escrita e execução os valores recebidos
como parâmetro.
• No método octal você deverá retornar a representação octal das permissões (multiplicar pelo
sua base e somar os valores).
```

### Estrutura Arquivo
```
• No método new você deverá atribuir ao campo permissao as permissões padrão (leitura =
false, escrita = true, execução = false). No restante dos campos você deverá preencher com
os valores recebidos como parâmetro.
• No método stat você deverá produzir uma saída no mesmo formato da Figura 1. Lembre-se que
aqui você utilizará os campos da estrutura autorreferenciada.
```

![image](https://github.com/user-attachments/assets/b79a1172-5585-4c5c-b1fe-8afeae6e90ce)

```
• No método alterar_permissao você deverá alterar a permissão do arquivo para a que foi
passada como parâmetro.
```

### Estrutura Diretorio
```
• No método new você deverá inicializar os campos da estrutura Diretorio com o que foi passado
como parâmetro do método.
• No método adiciona_arquivo você deve colocar um arquivo no vetor de arquivos da estrutura
autorreferenciada.
• No método remove_arquivo você remover um arquivo do vetor de arquivos da estrutura
autorreferenciada.
• No método listar_conteudo você deve listar todos os arquivos presentes no diretório
```
### Estrutura Usuario
```
• No método new você deverá inicializar os campos da estrutura Usuario com o que foi passado
como parâmetro do método.
• No método adiciona_grupo você deve adicionar um grupo ao vetor da estrutura Grupo na
autorreferencia de usuário.
• No método remove_grupo você deve remover um grupo ao vetor da estrutura Grupo na
autorreferencia de usuário.
• No método listar_grupos você deve listar todos os grupos daquele usuário.
```

### Estrutura Grupo
```
• No método new você deverá inicializar os campos da estrutura Grupo com o que foi passado
como parâmetro do método.
• No método adiciona_membro você deve adicionar um grupo ao vetor da estrutura Grupo na
autorreferencia de usuário.
• No método remove_membro você deve remover um grupo ao vetor da estrutura Grupo na
autorreferencia de usuário.
• No método listar_grupos você deve listar todos os membros daquele grupo.
```
# Menu:
![image](https://github.com/user-attachments/assets/bace8a39-3824-40d7-bf1c-17e57a123208)

# Cube-360 🧊

**Cube-360** é um motor de renderização 3D leve e de alto desempenho, rodando inteiramente no terminal, escrito em **Rust**.

Ele renderiza múltiplos cubos rotativos usando caracteres ASCII, apresentando uma implementação de **Z-Buffer** para gerenciamento de profundidade, **redimensionamento responsivo** do terminal e **cores ANSI**.


## 🚀 Funcionalidades

* **Projeção 3D Real:** Utiliza matrizes de rotação para projetar coordenadas 3D em uma tela de terminal 2D.
* **Z-Buffer:** Calcula a profundidade para renderizar corretamente superfícies sobrepostas.
* **Design Responsivo:** Detecta automaticamente o tamanho da janela do terminal e ajusta a área de renderização em tempo real (alimentado pela biblioteca `crossterm`).
* **Renderização Multicores:** Usa códigos de escape ANSI para diferenciar os cubos.
* **Alto Desempenho:** Otimizado para rodar suavemente a 60 FPS no modo *release*.

## 🛠️ Instalação e Uso

### Pré-requisitos
Você precisa ter **Rust** e **Cargo** instalados na sua máquina.

### Rodando o projeto
1.  Clone este repositório:
    ```bash
    git clone https://github.com/jjeancarlos/cube-360.git
    ````
2. Entre no diretório do projeto:
    ```bash
    cd cube-360
    ```
3.  Execute o projeto no modo **release** (recomendado para FPS suave):
    ```bash
    cargo run --release
    ```


## 💻 Detalhes Técnicos

Este projeto é uma portagem em Rust e aprimoramento das implementações matemáticas clássicas em C para renderização de objetos 3D no terminal.

* **Linguagem:** Rust 2021

* **Dependências:** `crossterm` (para manipulação do terminal e responsividade)

* **Matemática:** Matrizes de Rotação 3D (Yaw, Pitch, Roll) projetadas para 2D.



## 📝 Licença

Este projeto está licenciado sob a Licença MIT — veja o arquivo **[LICENSE](LICENSE)** para detalhes.
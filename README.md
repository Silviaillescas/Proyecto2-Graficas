# Proyecto2-Graficas

Raytracer con Animación de Texturas y Ciclo de Día/Noche
Este proyecto es una implementación de un Raytracer en Rust que incluye características avanzadas como animación de texturas, ciclo de día y noche, materiales personalizados y controles de cámara. El raytracer genera una escena tridimensional interactiva, con una fuente de luz que simula el sol y objetos que representan un muñeco sobre un suelo con textura de bloques.

# Características principales

Ciclo de día y noche: El sol se mueve a lo largo del cielo, cambiando su posición e intensidad de luz en función del tiempo transcurrido, simulando un ciclo de día y noche.
Animación de texturas: Algunas superficies tienen texturas animadas que varían a lo largo del tiempo, agregando dinamismo a la escena.
Textura de bloques en el suelo: El suelo tiene una textura personalizada que imita un patrón de bloques.
Materiales personalizados: Cada parte del muñeco y el entorno tiene un material único con su propia textura, albedo, reflectividad y parámetros de transparencia.
Skybox: El fondo de la escena es un cielo degradado, simulando un horizonte claro y un cielo azul profundo.
Controles de cámara: Se puede acercar y alejar la cámara, además de rotar alrededor del centro de la escena.
Sombras suaves: La luz del sol genera sombras suaves en los objetos, mejorando la sensación de profundidad y realismo.

# Requisitos
Para ejecutar este proyecto, es necesario tener instalados:
Rust (versión 1.55.0 o superior)
Cargo (para la gestión de dependencias en Rust)
Este proyecto no usa librerías externas que no sean parte del ecosistema oficial de Rust, como nalgebra_glm para operaciones matemáticas.

# Estructura del código
El proyecto está organizado en los siguientes módulos:

framebuffer.rs: Gestiona el búfer de la pantalla, donde se dibujan los píxeles.
ray_intersect.rs: Contiene la lógica para determinar las intersecciones de los rayos con los objetos.
sphere.rs y cube.rs: Definen la geometría de las esferas y cubos, que se utilizan en la escena.
color.rs: Maneja los colores de los píxeles, materiales y luces.
camera.rs: Controla la posición y la orientación de la cámara en la escena.
light.rs: Define las propiedades de las fuentes de luz, como la intensidad, el color y la posición.
material.rs: Gestiona las propiedades de los materiales, como el albedo, la reflectividad y las texturas.

# Cómo ejecutar
Clona este repositorio en tu máquina local:

git clone 
Entra en el directorio del proyecto:
cd raytracer
Compila y ejecuta el proyecto:
cargo run --release
Esto abrirá una ventana gráfica donde podrás ver la escena generada.

# Controles
W: Acercar la cámara hacia el centro.
S: Alejar la cámara del centro.
Flechas izquierda/derecha: Rotar la cámara horizontalmente alrededor del muñeco.
Flechas arriba/abajo: Rotar la cámara verticalmente

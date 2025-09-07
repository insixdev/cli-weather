# cli-weather

`cli-weather` es una aplicación de línea de comandos (CLI) para consultar el estado del tiempo de cualquier ciudad desde tu terminal de manera rápida y sencilla.

## Características

- Consulta el clima actual de cualquier ciudad.
- Muestra información como temperatura, humedad, viento y descripción general.
- Ligero y fácil de usar.
- Ideal para desarrolladores, sysadmins, y entusiastas de la terminal.

## Instalación

Clona el repositorio:

```bash
git clone https://github.com/insixdev/cli-weather.git
cd cli-weather
```

Compila la aplicación (ajusta según el lenguaje):

```bash
# Ejemplo para C/C++
make
# O para Python, asegúrate de tener las dependencias
pip install -r requirements.txt
```

## Uso

Ejecuta el programa desde la terminal indicando la ciudad:

```bash
./cli-weather [ciudad]
```



## Configuración

Algunas versiones pueden requerir una API Key para servicios de clima (como OpenWeatherMap). Consulta el archivo de configuración `.env` o las instrucciones dentro del código fuente para agregar tu clave.
Agregar su api key en el .env

## Requisitos

- Tener instalado [curl](https://curl.se/) o [wget] si la app lo utiliza.
- Internet activo.
- Dependencias adicionales según el lenguaje (ver `requirements.txt` o `Makefile`).




Hecho con ❤️ por [insixdev](https://github.com/insixdev)

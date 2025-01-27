use ureq::Error;

pub struct HTTPResponse { }

impl HTTPResponse {
  pub fn get_message(error: Error) -> String {
    match error {
      Error::Status(400, _) => String::from("Error en el servidor, verifique que se estén enviando todos los datos correctamente"),
      Error::Status(401, _) => String::from("No está autorizado para realizar esta acción, verifique sus credenciales"),
      Error::Status(403, _) => String::from("Acceso prohibido, no tiene los permisos necesarios"),
      Error::Status(404, _) => String::from("El recurso solicitado no fue encontrado"),
      Error::Status(500, _) => String::from("Error interno del servidor"),
      Error::Status(503, _) => String::from("Servicio no disponible temporalmente"), 
      _ => String::from("No se pudo completar la solicitud al servidor")
    }
  }
}

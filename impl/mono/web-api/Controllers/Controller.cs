using Newtonsoft.Json;

namespace web_api.Controllers
{
    public class Controller : Microsoft.AspNet.Mvc.Controller
    {
        protected T FromJson<T>(string data)
        {
            return JsonConvert.DeserializeObject<T>(data);
        }

        protected virtual string ToJson(object data)
        {
            return JsonConvert.SerializeObject(data);
        }
    }
}
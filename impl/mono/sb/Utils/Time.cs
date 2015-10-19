using System;

namespace sb.Utils
{
    public static class Time
    {
        public static string GetString()
        {
            return DateTime.Now.ToString("yyyyMMddhhmmss");
        }
    }
}
using System;

namespace cb.Utils
{
    public static class Time
    {
        public static string GetString()
        {
            return DateTime.Now.ToString("yyyyMMddhhmmss");
        }
    }
}
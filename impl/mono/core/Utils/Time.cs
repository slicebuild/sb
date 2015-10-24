using System;

namespace sb.Core.Utils
{
    public static class Time
    {
        public static string GetString()
        {
            return DateTime.Now.ToString("yyyyMMddhhmmss");
        }
    }
}
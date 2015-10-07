using System;
using System.Collections.Generic;
using System.IO;

namespace cb.Utils
{
    public static class Text
    {
        private static readonly HashSet<char> Digits = new HashSet<char>
        {
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' 
        };

        public static bool IsDigit(this char ch)
        {
            return Digits.Contains(ch);//todo:replace with ASCII code compare
        }

        public static bool IsEmpty(this string s)
        {
            return string.IsNullOrWhiteSpace(s);
        }

        public static string[] Split(this string text, string sep, bool removeEmpties = true)
        {
            return text.Split(new[] { sep }, removeEmpties ? StringSplitOptions.RemoveEmptyEntries : StringSplitOptions.None);
        }

        public static Stream ToStream(this string text)
        {
            MemoryStream stream = new MemoryStream();
            StreamWriter writer = new StreamWriter(stream);
            writer.Write(text);
            writer.Flush();
            stream.Position = 0;
            return stream;
        }

        public static string TrimQuotes(this string text)
        {
            if (text.StartsWith("\"") && text.EndsWith("\""))
            {
                return text.Substring(1, text.Length - 2);
            }
            return text;
        }
    }
}
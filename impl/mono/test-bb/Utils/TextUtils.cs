using System.Collections.Generic;

namespace sb.TestBB.Utils
{
    public static class TextUtils
    {
        public static bool ListContains(IList<string> list, string text)
        {
            foreach (var line in list)
            {
                if (line.Contains(text))
                    return true;
            }
            return false;
        }
    }
}
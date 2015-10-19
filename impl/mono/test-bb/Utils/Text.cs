using System.Collections;
using System.Collections.Generic;

namespace sb_bbt.Utils
{
    public static class Text
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
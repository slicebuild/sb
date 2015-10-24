using System;
using System.Diagnostics;
using System.Text;

namespace sb.Core.Utils
{
    public class SemVerNameParser
    {
        /// <summary>
        /// http://semver.org/
        /// supported version formats: 
        /// 1.0.1
        /// 1.0.2-beta
        /// 2.3.5-rc.1
        /// 4.3.2-alpha.1.2 - prerelase is two digits MAX!
        /// </summary>
        public static SemVerName Parse(string text, SemVerName parentSemVerName = null)
        {
            try
            {
                text = text.Trim();
                var sb = new StringBuilder();
                var dash = false;

                var name = "";
                var namesDone = false;

                var versions = new[] {0, 0, 0, 0, 0, 0};
                var versionIndex = 0;

                for (var i = 0; i < text.Length; i++)
                {
                    var ch = text[i];

                    if (ch == ' ')
                        continue;

                    if (dash && !char.IsDigit(ch))
                    {
                        sb.Append('-');
                        dash = false;
                    }

                    if (ch == '-' && !namesDone)
                    {
                        dash = true;
                        continue;
                    }

                    if (dash && char.IsDigit(ch))
                    {
                        name = sb.ToString();
                        namesDone = true;
                        dash = false;
                        sb = new StringBuilder();
                    }

                    if (ch == '.')
                        namesDone = true;

                    if (ch == '.' || ch == '-')
                    {
                        var verPart = ConvertVersion(sb.ToString());
                        versions[versionIndex] = verPart;
                        versionIndex++;
                        sb = new StringBuilder();
                        continue;
                    }

                    sb.Append(ch);
                }

                if (sb.Length > 0)
                {
                    var s = sb.ToString();
                    if (!namesDone)
                        name = s;
                    else
                        versions[versionIndex] = ConvertVersion(s);
                }

                return new SemVerName(
                    text,
                    name,
                    new Tuple<int, int, int, int, int, int>(versions[0], versions[1], versions[2], versions[3],
                        versions[4], versions[5]),
                    parentSemVerName?.ParentVersion);
            }
            catch (Exception ex)
            {
                Debug.WriteLine(ex);
                return null;
            }
        }

        private static int ConvertVersion(string txt)
        {
            switch (txt)
            {
                case "alpha":
                {
                    return -3;
                }
                case "beta":
                {
                    return -2;
                }
                case "rc":
                {
                    return -1;
                }
            }
            return int.Parse(txt);
        }
    }
}
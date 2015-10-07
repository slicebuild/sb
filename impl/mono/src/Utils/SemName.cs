using System.Text;

namespace cb.Utils
{
    public class SemName
    {
        /// <summary>
        /// Parse names with semantic versions, like 'my-app-1.2.3-beta.3' or 'your-cool-app-1.2' or just 'their-app' or simply 'app'
        /// </summary>
        /// <param name="name">Input name</param>
        public SemName(string name)
        {
            var chars = name.ToCharArray();
            var sbName = new StringBuilder();
            var sbVersion = new StringBuilder();
            var sbPreRelease = new StringBuilder();
            var mode = 1; // 1=name, 2=version, 3=pre-release

            for (var i = 0; i < chars.Length; i++)
            {
                var ch = chars[i];
                var chNext = chars.Length > i + 1 ? chars[i + 1] : ' ';

                if (mode == 1 && ch == '-' && chNext.IsDigit())
                {
                }
                else if (mode == 1 && chNext != '.')
                {
                    sbName.Append(ch);
                }
                else if (mode == 1 && chNext == '.')
                {
                    mode = 2;
                    sbVersion.Append(ch);
                }
                else if (mode == 2 && ch != '-')
                {
                    sbVersion.Append(ch);
                }
                else if (mode == 2 && ch == '-')
                {
                    mode = 3;
                }
                else if (mode == 3)
                {
                    sbPreRelease.Append(ch);
                }
            }
            NamePart = sbName.ToString();
            VersionPart = sbVersion.ToString().Trim();
            PreReleasePart = sbPreRelease.ToString().Trim();
        }

        public string NamePart { get; set; }
        public string VersionPart { get; set; }
        public string PreReleasePart { get; set; }

        public override string ToString()
        {
            var sb = new StringBuilder();
            if (!NamePart.IsEmpty())
                sb.Append(NamePart);
            if (!VersionPart.IsEmpty())
            {
                if (!NamePart.IsEmpty())
                    sb.Append("-");
                sb.Append(VersionPart);
                if (!PreReleasePart.IsEmpty())
                {
                    sb.Append("-").Append(PreReleasePart);
                }
            }
            return sb.ToString();
        }
    }
}
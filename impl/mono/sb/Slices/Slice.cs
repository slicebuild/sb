using System.Collections.Generic;
using System.Linq;
using sb.Utils;

namespace sb.Slices
{
    public class Slice
    {
        public Slice(string relPath, SemVerName semVerName, IList<string> lines)
        {
            RelPath = relPath;
            SemVerName = semVerName;
            Sections = new List<SliceSection>();
            OsList = new List<string>();

            int lineStart = 0;
            SliceSection section;
            while ((section = SliceSection.Parse(lines, lineStart)) != null)
            {
                lineStart = section.EndLine > lineStart ? section.EndLine : lineStart + 1;
                Sections.Add(section);
            }

            foreach (var s in Sections.Where(s => s.SectionType == SliceSection.Type.OS))
            {
                foreach (var line in s.Lines.Where(l => !l.StartsWith("#")))
                {
                    var svn = SemVerNameParser.Parse(line);
                    OsList.Add(svn.Name);
                }
            }
        }

        public string RelPath { get; set; }
        public SemVerName SemVerName { get; }
        public IList<SliceSection> Sections { get; } 
        public IList<string> OsList { get; }

        public override bool Equals(object obj)
        {
            var other = (Slice) obj;
            return SemVerName.Equals(other.SemVerName);
        }

        public override int GetHashCode()
        {
            return SemVerName.GetHashCode();
        }

        public override string ToString()
        {
            return SemVerName.ToString();
        }
    }
}
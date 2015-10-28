using System.Collections.Generic;
using System.Linq;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class Slice
    {
        public Slice(string relPath, SemVerInfo info, IList<string> lines)
        {
            RelPath = relPath;
            Info = info;
            Sections = new List<SliceSection>();

            int lineStart = 0;
            SliceSection section;
            while ((section = SliceSection.Parse(lines, lineStart)) != null)
            {
                lineStart = section.EndLine > lineStart ? section.EndLine : lineStart + 1;
                Sections.Add(section);
            }

            foreach (var s in Sections.Where(s => s.SectionType == SliceSection.Type.DEP))
            {
                foreach (var line in s.Lines.Where(l => !l.StartsWith("#")))
                {
                    var depInfo = new SemVerInfo(line);
                    if (!DepInfos.Contains(depInfo))
                        DepInfos.Add(depInfo);
                }
            }            
        }

        public string RelPath { get; set; }
        public SemVerInfo Info { get; }
        public IList<SliceSection> Sections { get; } 
        public List<SemVerInfo> DepInfos { get; } = new List<SemVerInfo>();

        public override bool Equals(object obj)
        {
            var other = (Slice) obj;
            return Info.Equals(other.Info);
        }

        public override int GetHashCode()
        {
            return Info.GetHashCode();
        }

        public override string ToString()
        {
            return Info.ToString();
        }
    }
}
using System.Collections.Generic;
using System.Linq;
using cb.Utils;

namespace cb.Slices
{
    public class Slice
    {
        public Slice(SemName semName, IList<string> lines)
        {
            SemName = semName;
            SemVersion = new SemVersion(semName.VersionPart, semName.PreReleasePart);
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
                    OsList.Add(new SemName(line.Trim()).NamePart.ToLower());
                }
            }
        }

        public string Name => SemName.NamePart;
        public SemName SemName { get; }        
        public SemVersion SemVersion { get; }        
        public IList<SliceSection> Sections { get; } 
        public IList<string> OsList { get; }        
    }
}
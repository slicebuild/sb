using System;
using System.Collections.Generic;

namespace cb.Slices
{
    public class SliceSection
    {
        public enum Type
        {
            None,
            OS,
            DEP,
            ADD,
            CMD,
            COPY,
            ENTRYPOINT,
            ENV,
            EXPOSE,
            FROM,
            LABEL,
            MAINTAINER,
            ONBUILD,
            RUN,
            USER,
            VOLUME,
            WORKDIR
        }

        public static SliceSection Parse(IList<string> lines, int lineStart)
        {
            var section = new SliceSection
            {
                Lines = new List<string>()
            };

            for (var pos = lineStart; pos < lines.Count; pos++)
            {
                var line = lines[pos].Trim();
                Type foundType;
                Enum.TryParse(line, out foundType);

                if (foundType != Type.None && section.SectionType == Type.None)
                {
                    section.StartLine = pos;
                    section.SectionType = foundType;
                }
                else if (foundType != Type.None && section.SectionType != Type.None)
                {                    
                    break;
                }
                else if (section.SectionType != Type.None && line.Length > 0)
                {
                    section.Lines.Add(line);
                    section.EndLine = pos;
                }
            }

            return section.SectionType != Type.None ? section : null;
        } 

        public Type SectionType { get; private set; }
        public int StartLine { get; private set; }
        public int EndLine { get; private set; }
        public IList<string> Lines { get; private set; } 
    }
}
using System;
using System.Linq;
using System.Text;
using sb.Core.Slices;

namespace sb.Core.Formatters.Docker
{
    public class FormatterDocker : IFormatter
    {
        public void Write(Slice slice, StringBuilder sb)
        {
            foreach (var section in slice.Sections)
            {
                Write(section, sb);
            }
        }

        public void Write(SliceSection section, StringBuilder sb)
        {
            if (section.SectionType != SliceSection.Type.RUN)
            {
                foreach (var line in section.Lines.Where(item => !item.StartsWith("#")))
                {
                    sb.Append(section.SectionType).Append(" ").AppendLine(line);
                }
            }

            if (section.SectionType == SliceSection.Type.RUN)
            {
                var text = string.Join($" && {"\\"} {Environment.NewLine}", section.Lines.Where(item => !item.StartsWith("#")));
                sb.Append("RUN ");
                sb.AppendLine(text);
            }
        }
    }
}
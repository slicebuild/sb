using System;
using System.Text;
using cb.Slices;

namespace cb.Formatters.Docker
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
            if (section.SectionType == SliceSection.Type.FROM)
            {
                foreach (var line in section.Lines)
                {
                    sb.Append("FROM ").AppendLine(line);
                }
            }

            if (section.SectionType == SliceSection.Type.RUN)
            {
                var text = string.Join($" && {"\\"} {Environment.NewLine}", section.Lines);
                sb.Append("RUN ");
                sb.AppendLine(text);
            }
        }
    }
}
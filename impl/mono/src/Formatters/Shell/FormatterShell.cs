using System.Text;
using cb.Slices;

namespace cb.Formatters.Shell
{
    public class FormatterShell : IFormatter
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
            if (section.SectionType == SliceSection.Type.OS)
            {
                foreach (var line in section.Lines)
                {
                    sb.AppendLine("# " + line);
                }
            }

            if (section.SectionType == SliceSection.Type.RUN)
            {
                foreach (var line in section.Lines)
                {
                    sb.AppendLine(line);
                }
            }
        }
    }
}
using System.Text;
using sb.Slices;

namespace sb.Formatters
{
    public interface IFormatter
    {
        void Write(Slice slice, StringBuilder sb);
        void Write(SliceSection section, StringBuilder sb);
    }
}
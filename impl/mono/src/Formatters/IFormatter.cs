using System.Text;
using cb.Slices;

namespace cb.Formatters
{
    public interface IFormatter
    {
        void Write(Slice slice, StringBuilder sb);
        void Write(SliceSection section, StringBuilder sb);
    }
}
import os
import subprocess

__file_dir__ = os.path.realpath(os.path.join(__file__, '..'))

LOGITECH_LED_SDK = os.environ['LOGITECH_LED_SDK']

def generate_bindings(arch):
    # Logitech only provides MSVC libraries
    target_triple = f'{arch}-pc-windows-msvc'
    
    subprocess.run(
        [
            'bindgen',
            'wrapper.h',
            '-o', f'src/bindings-{arch}.rs',
            '--allowlist-type', 'LogiLed::.*',
            '--allowlist-function', 'Logi.*',
            '--allowlist-var', '.*',
            # TODO: This should be enumerated manually to insulate from ABI changes.
            '--rustified-enum', 'LogiLed::.*',
            '--',
            '-xc++',
            f'-I{LOGITECH_LED_SDK}//Include',
            '-target', target_triple
        ],
        cwd=__file_dir__,
        check=True
    )

def main():
    # Logitech only provides libraries for x64 and x86.
    # Only use i686 for simplicty; can LGS or LGHUB even run on anything less?
    for arch in ['x86_64', 'i686']:
        generate_bindings(arch)
    
if __name__ == '__main__':
    main()
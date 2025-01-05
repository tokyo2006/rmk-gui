<template>
    <div class="flex justify-center items-center h-screen">
        <div class="flex flex-col w-[650px] gap-1">
            <div class="flex justify-between w-full">
                <Key lower="Esc" />
                <Key lower="F1" />
                <Key lower="F2" />
                <Key lower="F3" />
                <Key lower="F4" />
                <Key lower="F5" />
                <Key lower="F6" />
                <Key lower="F7" />
                <Key lower="F8" />
                <Key lower="F9" />
                <Key lower="F10" />
                <Key lower="F11" />
                <Key lower="F12" />
            </div>

            <div class="flex justify-between w-full">
                <Key lower="`" upper="~" />
                <Key lower="1" upper="!" />
                <Key lower="2" upper="@" />
                <Key lower="3" upper="#" />
                <Key lower="4" upper="$" />
                <Key lower="5" upper="%" />
                <Key lower="6" upper="^" />
                <Key lower="7" upper="&" />
                <Key lower="8" upper="*" />
                <Key lower="9" upper="(" />
                <Key lower="0" upper=")" />
                <Key lower="-" upper="-" />
                <Key lower="=" upper="+" />
                <Key lower="BACKSPACE" length=2 />
            </div>

            <div class="flex justify-between w-full">
                <Key lower="TAB" length=1.5 />
                <Key lower="Q" />
                <Key lower="W" />
                <Key lower="E" />
                <Key lower="R" />
                <Key lower="T" />
                <Key lower="Y" />
                <Key lower="U" />
                <Key lower="I" />
                <Key lower="O" />
                <Key lower="P" />
                <Key lower="[" upper="{" />
                <Key lower="]" upper="}" />
                <Key lower="\" upper="|" length=1.5 />
            </div>

            <div class="flex justify-between w-full">
                <Key lower="CAPSLOCK" length=1.75 />
                <Key lower="A" />
                <Key lower="S" />
                <Key lower="D" />
                <Key lower="F" />
                <Key lower="G" />
                <Key lower="H" />
                <Key lower="J" />
                <Key lower="K" />
                <Key lower="L" />
                <Key lower=";" upper=":" />
                <Key lower="'" upper="'" />
                <Key lower="ENTER" length=2.25 />
            </div>

            <div class="flex justify-between w-full">
                <Key lower="SHIFT" length=2.25 />
                <Key lower="Z" />
                <Key lower="X" />
                <Key lower="C" />
                <Key lower="V" />
                <Key lower="B" />
                <Key lower="N" />
                <Key lower="M" />
                <Key lower="," upper="<" />
                <Key lower="." upper=">" />
                <Key lower="/" upper="?" />
                <Key lower="SHIFT" length=2.75 />
            </div>

            <div class="flex justify-between w-full">
                <Key lower="CTRL" length=1.25 />
                <Key lower="WIN" length=1.25 />
                <Key lower="ALT" length=1.25 />
                <Key lower="" length=6.25 />
                <Key lower="ALT" length=1.25 />
                <Key lower="WIN" length=1.25 />
                <Key lower="MENU" length=1.25 />
                <Key lower="CTRL" length=1.25 />
            </div>
        </div>
    </div>
</template>
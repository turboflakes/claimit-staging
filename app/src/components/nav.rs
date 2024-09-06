use crate::components::{
    buttons::{NetworkProviderIconButton, NetworkSubscriber},
    spinners::Spinner,
};
use crate::state::StateContext;
use claimit_common::runtimes::support::SupportedRelayRuntime;
use num_format::{Locale, ToFormattedString};
use yew::{function_component, html, use_context, Callback, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct NavbarProps {
    pub runtime: SupportedRelayRuntime,
    pub ontoggle_provider: Callback<()>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    html! {
        <nav class="fixed w-full z-20 top-0 start-0 bg-gradient-to-l from-white from-20%">
            <div class="flex items-center justify-between sm:justify-end px-2 sm:p-4 h-12">
                <a class="block sm:hidden" href="https://goclaimit.app/">
                    <svg class="w-8 h-8 text-gray-800" viewBox="0 0 124 124" version="1.1" xmlns="http://www.w3.org/2000/svg">
                        <circle fill="#FFFFFF" cx="62" cy="62" r="62"></circle>
                        <path d="M38.7226546,75.8850399 C37.9793726,75.8829489 37.2522135,76.0712277 36.5482332,76.477671 C35.8442529,76.8841142 35.3176191,77.419713 34.9478041,78.0644683 C34.3460898,79.1135294 34.3452726,80.1649023 34.9536178,81.2185871 C36.1617337,83.3111053 37.4997517,85.1131369 38.9604592,86.6302038 C40.4420638,88.168974 42.043857,89.4585116 43.7630228,90.502363 C45.4762414,91.5426033 47.2787833,92.3606279 49.1703686,92.9570555 C51.0392774,93.5463329 52.9737589,93.9375933 54.9740612,94.1296257 C56.9434473,94.3186902 58.9554038,94.3331798 61.0099449,94.1717161 C63.0408627,94.012109 65.0888588,93.6942058 67.1537867,93.2170465 C69.1848715,92.7477077 71.2145826,92.1621487 73.2428195,91.4600194 C75.267374,90.7591649 77.2750862,89.9438883 79.2658432,89.0139104 C81.2447236,88.0894805 83.1641191,87.0902751 85.0240942,86.016418 C86.0438978,85.4276341 86.5918936,84.5619477 86.7142025,83.448753 C86.8075357,82.59928 86.6412482,81.7670899 86.1752959,80.9600368 C85.7093435,80.1529837 85.0717895,79.5928794 84.2894577,79.2489719 C83.2642484,78.7982971 82.2405441,78.8400322 81.2207404,79.4288161 C78.9970659,80.7126552 76.8097482,81.8589027 74.6584986,82.8670028 C72.5392844,83.8600907 70.4613852,84.68786 68.4242457,85.3488925 C66.4110469,86.0021563 64.4670346,86.4895829 62.5916218,86.8087445 C60.7627968,87.1199778 59.0010933,87.2571854 57.3062584,87.2166502 C55.6559584,87.1771801 54.0705205,86.9552938 52.5503863,86.5479838 C51.0590794,86.1483979 49.6552056,85.561329 48.3391074,84.7859621 C47.0211984,84.0095283 45.7955391,83.0223835 44.6581348,81.8296163 C43.4871426,80.6016261 42.4204411,79.1457624 41.4512947,77.4671516 C40.8429495,76.4134668 39.9320252,75.8884881 38.7226546,75.8850399 Z M88.9131678,69.6213752 C87.9411858,69.4886301 83.8790854,68.9683036 81.593556,68.1327337 C79.4422447,67.3462328 77.4761679,66.2546558 75.6950398,64.8585038 C74.2638145,65.7240509 72.8652103,66.4577754 71.4997629,67.0607003 C69.6278806,67.8872457 67.7314912,68.4821109 65.8110441,68.8467528 C63.9097533,69.2077573 61.8761775,69.3895165 59.7100667,69.3895165 C56.9732425,69.3895165 54.3957831,68.9717718 51.9768679,68.1410332 C49.5579791,67.3103037 47.3470744,66.1370085 45.3435992,64.6221858 C45.2263063,64.5335009 45.1099768,64.4438525 44.9946104,64.3532409 C43.8116855,65.1047255 42.5607618,65.7775263 41.2013407,66.398642 L40.8946245,66.5370801 C38.9018224,67.4256353 38.4320565,67.8628651 34.7360484,68.6261908 C31.0400403,69.3895165 29.0949829,62.6085194 31.0837268,61.9823022 C33.0724707,61.356085 34.6554136,61.2160502 36.8175496,60.4183689 C37.9642283,59.9953224 39.0295799,59.5222608 40.0131006,58.9981571 C38.6543173,56.9921596 37.5994226,54.8059911 36.8495189,52.4391074 C36.0709158,49.9816412 35.6799395,47.3905302 35.6799395,44.6652417 C35.6799395,42.9791718 35.8798643,41.3801598 36.2765535,39.8677825 C36.6812789,38.3247668 37.2780075,36.9346726 38.0620282,35.695414 C38.8544103,34.4429391 39.8012313,33.3704558 40.9002178,32.4759319 C42.0048691,31.5767971 43.2251468,30.8832259 44.5609987,30.3951262 C45.8992764,29.9061401 47.3401866,29.6590331 48.884902,29.6590331 L49.1045012,29.6602371 C51.9283814,29.6908521 54.2161243,30.3101637 55.9884781,31.4584493 C57.8512243,32.6652989 59.1987211,34.2550664 60.0379292,36.2219603 C60.843982,38.1111466 61.2498122,40.1384804 61.2498122,42.3051399 C61.2498122,44.0121576 60.9400407,45.7702623 60.3123011,47.5781522 C59.7026464,49.3339578 58.8624164,51.0537631 57.7893704,52.7364944 C56.7341004,54.3913495 55.5113256,55.9865516 54.1203645,57.5216817 C53.6065719,58.1081188 53.0741115,58.6758879 52.5229849,59.2249909 C53.1312345,59.6027516 53.7462898,59.9291427 54.3824525,60.2145432 L54.5917336,60.3066338 C56.4617154,61.1135534 58.3524802,61.5204071 60.2653847,61.5204071 C62.3835688,61.5204071 64.1384647,61.3821471 65.528801,61.0952523 C66.8731685,60.8178431 68.1009861,60.4037646 69.2107844,59.8488654 C69.6495591,59.6294781 70.1072032,59.3911623 70.583741,59.1339641 C69.310885,57.0826027 68.3239406,54.8403401 67.6250352,52.4062213 C66.9224111,49.9591511 66.5695069,47.3789788 66.5695069,44.6652417 C66.5695069,42.9791718 66.7694318,41.3801598 67.1661209,39.8677825 C67.5693154,38.3306034 68.1512142,36.9457536 68.9070788,35.7111748 C69.6776168,34.4526293 70.6160462,33.3748005 71.7203705,32.4759319 C72.8250218,31.5767971 74.0452995,30.8832259 75.3811514,30.3951262 C76.7194291,29.9061401 78.1603393,29.6590331 79.7050547,29.6590331 L79.9246538,29.6602371 C82.748534,29.6908521 85.0362769,30.3101637 86.8086308,31.4584493 C88.6713769,32.6652989 90.0188738,34.2550664 90.8580819,36.2219603 C91.6641347,38.1111466 92.0699649,40.1384804 92.0699649,42.3051399 C92.0699649,44.0121576 91.7601933,45.7702623 91.1324538,47.5781522 C90.522799,49.3339578 89.6825691,51.0537631 88.609523,52.7364944 C87.5542531,54.3913495 86.3314782,55.9865516 84.940637,57.5215451 C84.3014948,58.2510624 83.6334662,58.9516931 82.936551,59.6234374 C83.2639287,59.8333106 83.5983723,60.0281146 83.9399099,60.2078056 L84.1284667,60.3051106 C85.1910357,60.8428149 86.4644845,61.2479137 87.9488131,61.5204071 C89.0649633,61.7127348 90.4386806,61.9628637 92.0699649,62.2707938 C94.5168914,62.7326888 94.103848,66.7915609 92.0699649,69.3895165 C91.4902715,70.1299808 89.8851498,69.7541204 88.9131678,69.6213752 Z M79.423858,37.666972 C78.8849452,37.666972 78.3911371,37.7805614 77.8535593,38.045841 C77.3754563,38.3053389 76.963809,38.6989037 76.6052805,39.2110873 C76.1934934,39.7993546 75.8766944,40.5179094 75.6416325,41.3602145 C75.3908446,42.2588713 75.2715934,43.3149057 75.2715934,44.5264122 C75.2715934,46.9993262 75.6685244,49.2781317 76.4713264,51.3610773 C76.8510895,52.3464085 77.290399,53.271376 77.8262078,54.1264555 C78.1993499,53.751651 78.5549096,53.3742274 78.8928393,52.9942426 C79.8593649,51.8959433 80.6839569,50.7872906 81.3649194,49.6669975 C82.0231774,48.5840568 82.5230293,47.4589768 82.8627973,46.2910243 C83.1997314,45.1328133 83.3678784,43.9429841 83.3678784,42.7216285 C83.3678784,40.8029452 83.1176235,39.4002185 82.360607,38.6621274 C81.6486678,37.9679867 80.6731912,37.666972 79.423858,37.666972 Z M48.5027384,37.666972 C47.9638256,37.666972 47.4700175,37.7805614 46.9324397,38.045841 C46.4543367,38.3053389 46.0426894,38.6989037 45.6841609,39.2110873 C45.2723738,39.7993546 44.9555748,40.5179094 44.7205129,41.3602145 C44.469725,42.2588713 44.3504738,43.3149057 44.3504738,44.5264122 C44.3504738,46.9993262 44.7474048,49.2781317 45.5502068,51.3610773 C45.9299699,52.3464085 46.3692794,53.271376 46.9050882,54.1264555 C47.2782303,53.751651 47.63379,53.3742274 47.9717197,52.9942426 C48.9382453,51.8959433 49.7628373,50.7872906 50.4437998,49.6669975 C51.1020578,48.5840568 51.6019097,47.4589768 51.9416777,46.2910243 C52.2786118,45.1328133 52.4467588,43.9429841 52.4467588,42.7216285 C52.4467588,40.8029452 52.1965039,39.4002185 51.4394874,38.6621274 C50.7275482,37.9679867 49.7520716,37.666972 48.5027384,37.666972 Z" fill="currentColor" fill-rule="nonzero"></path>
                    </svg>
                </a>

                <div class="inline-flex items-center">

                    {
                        if state.network.is_ligh_client() && state.network.is_initializing() {
                            html! {
                                <p class="hidden sm:flex text-xs">{"Synchronizing light client..."}</p>
                            }
                        } else {
                            html! {}
                        }

                    }

                    <Spinner is_visible={state.network.is_fetching()} />

                    <div class="ms-4 inline-flex items-center space-x-2 text-gray-900">
                        {
                            if state.network.finalized_block_number.is_some() {
                                html! {
                                    <span class="text-sm">{format!("#{}", state.network.finalized_block_number.unwrap().to_formatted_string(&Locale::en))}</span>
                                }
                            } else { html! {} }
                        }

                        {
                            match props.runtime.clone() {
                                SupportedRelayRuntime::Polkadot => html! {
                                    <>
                                        <img class="h-8" src="/images/polkadot_icon.svg" alt="polkadot logo" />
                                        <span>{"Polkadot"}</span>
                                    </>
                                },
                                SupportedRelayRuntime::Kusama => html! {
                                    <>
                                        <img class="h-8" src="/images/kusama_icon.svg" alt="kusama logo" />
                                        <span>{"Kusama"}</span>
                                    </>
                                },
                                SupportedRelayRuntime::Rococo => html! {
                                    <>
                                        <img class="h-8" src="/images/rococo_icon.svg" alt="rococo logo" />
                                        <span>{"Rococo"}</span>
                                    </>
                                },
                            }
                        }
                    </div>

                    <NetworkProviderIconButton class="ms-4" onclick={props.ontoggle_provider.clone()} />

                </div>
            </div>
        </nav>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct FooterProps {
    pub runtime: SupportedRelayRuntime,
    pub onchange: Callback<SupportedRelayRuntime>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let state = use_context::<StateContext>().unwrap();
    let onchange = props.onchange.reform(move |e| e);

    html! {
        <footer class="w-full my-8 sm:my-2 dark:bg-gray-900">
            <div class="grid grid-cols-1 2xl:grid-cols-3 gap-4 content-center">
                <div class="inline-flex items-center justify-center 2xl:justify-start col-span-2 sm:mt-0 order-last 2xl:order-first">
                    <span class="inline-flex text-xs text-gray-500 sm:text-center dark:text-gray-400">
                        <span class="me-1">{"© 2024 Claimit · Built by"}</span>
                        <a href="https://turboflakes.io/" target="_blank" class="flex items-center hover:underline hover:underline-offset-2 hover:text-gray-900">
                            <span class="me-4">{"Turboflakes"}</span>
                            <svg class="w-4 h-4" viewBox="0 0 60 60" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                                <circle fill-rule="evenodd" cx="30" cy="30" r="30"></circle>
                                <path d="M20,46 C17.2399393,46 15,43.7600607 15,41 C15,38.2399393 17.2399393,36 20,36 C22.7600607,36 25,38.2399393 25,41 C25,43.7600607 22.7600607,46 20,46 Z M32.0751821,35 C29.3566227,35 18,30 18,30 C18,30 29.3566227,25 32.0751821,25 C34.7937414,25 37,27.2399393 37,30 C37,32.7600607 34.7974808,35 32.0751821,35 Z M39.777954,24 C36.8953212,24 20,19 20,19 C20,19 36.8913561,14 39.777954,14 C42.6645519,14 45,16.2399393 45,19 C45,21.7600607 42.6645519,24 39.777954,24 Z" fill="#EAEDF0"></path>
                            </svg>
                        </a>
                    </span>
                    <a href="https://github.com/turboflakes/claimit" target="_blank" class="text-gray-500 hover:text-gray-900 dark:hover:text-white ms-4">
                        <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 .333A9.911 9.911 0 0 0 6.866 19.65c.5.092.678-.215.678-.477 0-.237-.01-1.017-.014-1.845-2.757.6-3.338-1.169-3.338-1.169a2.627 2.627 0 0 0-1.1-1.451c-.9-.615.07-.6.07-.6a2.084 2.084 0 0 1 1.518 1.021 2.11 2.11 0 0 0 2.884.823c.044-.503.268-.973.63-1.325-2.2-.25-4.516-1.1-4.516-4.9A3.832 3.832 0 0 1 4.7 7.068a3.56 3.56 0 0 1 .095-2.623s.832-.266 2.726 1.016a9.409 9.409 0 0 1 4.962 0c1.89-1.282 2.717-1.016 2.717-1.016.366.83.402 1.768.1 2.623a3.827 3.827 0 0 1 1.02 2.659c0 3.807-2.319 4.644-4.525 4.889a2.366 2.366 0 0 1 .673 1.834c0 1.326-.012 2.394-.012 2.72 0 .263.18.572.681.475A9.911 9.911 0 0 0 10 .333Z" clip-rule="evenodd"/>
                        </svg>
                        <span class="sr-only">{"GitHub account"}</span>
                    </a>
                </div>
                <div class="flex items-center justify-center 2xl:justify-end sm:mt-0">
                    <NetworkSubscriber selected={props.runtime.clone()} disabled={state.network.is_busy()} {onchange} />
                </div>
            </div>
        </footer>
    }
}
